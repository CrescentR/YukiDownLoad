"""Douyin request adapter shared by source and managed yt-dlp runtimes."""

from yt_dlp.extractor.tiktok import DouyinIE
from yt_dlp.utils import ExtractorError
from yt_dlp.utils.traversal import traverse_obj


class YukiDouyinIE(DouyinIE, plugin_name='yuki'):
    def _real_extract(self, url):
        parser = getattr(self, '_parse_aweme_video_app', None)
        if not callable(parser):
            return super()._real_extract(url)

        video_id = self._match_id(url)
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

        detail = traverse_obj(self._download_json(
            'https://www.douyin.com/aweme/v1/web/aweme/detail/', video_id,
            'Downloading web detail JSON', 'Failed to download web detail JSON',
            headers=headers, query=query, fatal=False), ('aweme_detail', {dict}))
        if not detail:
            raise ExtractorError(
                'Fresh cookies (not necessarily logged in) are needed. '
                'Open this Douyin video in your browser, complete verification, '
                'and import fresh Douyin cookies. The site may also require '
                'a browser-generated request signature; cookies alone may not suffice.',
                expected=True)

        return parser(detail)
