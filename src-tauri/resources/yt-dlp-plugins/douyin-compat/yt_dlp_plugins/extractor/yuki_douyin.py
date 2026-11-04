"""Douyin request adapter shared by source and managed yt-dlp runtimes."""

from yt_dlp.extractor.tiktok import DouyinIE
from yt_dlp.utils import ExtractorError
from yt_dlp.utils.traversal import traverse_obj

# The share page renders for mobile browsers only.
_SHARE_UA = ('Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15 '
             '(KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1')


class YukiDouyinIE(DouyinIE, plugin_name='yuki'):
    def _real_extract(self, url):
        parser = getattr(self, '_parse_aweme_video_app', None)
        if not callable(parser):
            return super()._real_extract(url)

        video_id = self._match_id(url)
        detail = self._web_detail(url, video_id) or self._share_page_detail(video_id)
        if not detail:
            raise ExtractorError(
                'Fresh cookies (not necessarily logged in) are needed. '
                'Open this Douyin video in your browser, complete verification, '
                'and import fresh Douyin cookies. The site may also require '
                'a browser-generated request signature; cookies alone may not suffice.',
                expected=True)

        return parser(detail)

    def _web_detail(self, url, video_id):
        query = {'aweme_id': video_id}
        headers = {'Referer': url}
        cookies = self._get_cookies(getattr(self, '_WEBPAGE_HOST', 'https://www.douyin.com'))
        # Observed on the web detail request: the visitor cookie is also sent
        # as a request header and query parameter. Never manufacture a session.
        visitor = cookies.get('UIFID')
        if not visitor or not visitor.value:
            visitor = cookies.get('UIFID_TEMP')
        if visitor and visitor.value:
            headers['uifid'] = query['uifid'] = visitor.value

        return traverse_obj(self._download_json(
            'https://www.douyin.com/aweme/v1/web/aweme/detail/', video_id,
            'Downloading web detail JSON', 'Failed to download web detail JSON',
            headers=headers, query=query, fatal=False), ('aweme_detail', {dict}))

    def _share_page_detail(self, video_id):
        """Read the item embedded in the public share page.

        The web detail API now rejects requests without a browser-generated
        signature, while the share page only needs the visitor `ttwid` cookie,
        which users import with their Douyin cookies.
        """
        share_url = f'https://www.iesdouyin.com/share/video/{video_id}/'
        if not self._get_cookies(share_url).get('ttwid'):
            ttwid = self._get_cookies('https://www.douyin.com').get('ttwid')
            if not ttwid or not ttwid.value:
                return None
            self._set_cookie('.iesdouyin.com', 'ttwid', ttwid.value)

        webpage = self._download_webpage(
            share_url, video_id, 'Downloading share page', 'Failed to download share page',
            headers={'User-Agent': _SHARE_UA}, fatal=False)
        if not webpage:
            return None
        data = self._search_json(
            r'window\._ROUTER_DATA\s*=', webpage, 'router data', video_id, default={})
        item = traverse_obj(data, (
            'loaderData', lambda key, _: key.startswith('video_'), 'videoInfoRes',
            'item_list', 0, {dict}, any))
        # The share page sends `null` for empty fields (e.g. `video.bit_rate`)
        # where the app API omits them; the shared parser expects the latter.
        return _drop_nulls(item) if item else None


def _drop_nulls(value):
    if isinstance(value, dict):
        return {key: _drop_nulls(item) for key, item in value.items() if item is not None}
    if isinstance(value, list):
        return [_drop_nulls(item) for item in value]
    return value
