(async () => {
  const wait = () => new Promise(resolve => setTimeout(resolve, 160));
  const check = (value, message) => { if (!value) throw new Error(message); };
  const click = selector => { const node = document.querySelector(selector); check(node, selector); node.click(); };
  const input = document.querySelector('[data-url-input]');
  input.value = 'https://www.bilibili.com/video/BV1R1e4zKEh1/';
  input.dispatchEvent(new Event('input', {bubbles:true}));
  click('[data-action="analyze"]'); await wait(); await wait();
  check(document.querySelector('.parser-notice')?.textContent.includes('1080p'), 'Current resolution is visible');
  click('.parser-notice [data-action="open-cookie-settings"]'); await wait();
  check(document.querySelector('[data-setting="cookiesBrowser"]'), 'Direct navigation to network settings');
  check(!document.querySelector('.parser-dialog'), 'Parser does not overlap settings');
  const set = (key, value) => {
    const node = document.querySelector(`[data-setting="${key}"]`);
    node.value = value; node.dispatchEvent(new Event('input', {bubbles:true}));
  };
  set('cookiesBrowser', 'chrome');
  set('cookieText', 'SESSDATA=ui-fixture-only; bili_jct=fixture');
  click('[data-action="save-settings"]'); await wait(); await wait();
  check(!document.querySelector('.modal'), 'Save finishes');
  click('[data-action="open-settings"]'); await wait();
  click('[data-action="settings-tab"][data-tab="network"]'); await wait();
  check(document.querySelector('[data-setting="cookiesFile"]').value.includes('cookies.txt'), 'Imported file selected');
  check(document.querySelector('[data-setting="cookiesBrowser"]').value === '', 'Browser extraction disabled after paste import');
  check(document.querySelector('[data-setting="cookieText"]').value === '', 'Raw Cookie cleared after save');
  set('cookieSite', 'douyin');
  set('cookieText', 'ttwid=ui-visitor-fixture; __ac_nonce=ui-nonce');
  click('[data-action="save-settings"]'); await wait(); await wait();
  check(!document.querySelector('.modal'), 'Douyin Cookie import finishes');
  click('[data-action="open-settings"]'); await wait();
  click('[data-action="settings-tab"][data-tab="network"]'); await wait();
  check(document.querySelector('[data-setting="cookieText"]').value === '', 'Douyin Cookie cleared after save');
  set('cookieSite', 'youtube');
  check(document.querySelector('[data-setting="cookieSite"]').value === 'youtube', 'YouTube is selectable');
  set('cookieText', 'SID=ui-youtube-fixture; __Secure-3PAPISID=ui-fixture');
  click('[data-action="save-settings"]'); await wait(); await wait();
  check(!document.querySelector('.modal'), 'YouTube Cookie import finishes');
  click('[data-action="open-settings"]'); await wait();
  click('[data-action="settings-tab"][data-tab="network"]'); await wait();
  set('cookieSite', 'custom');
  check(document.querySelector('[data-setting="cookieDomain"]'), 'Custom domain field appears');
  set('cookieDomain', 'https://video.example.com/watch/123');
  set('cookieText', 'session.id=ui-custom-fixture');
  click('[data-action="save-settings"]'); await wait(); await wait();
  check(!document.querySelector('.modal'), 'Custom Cookie import finishes');
  const moduleUrl = suffix => performance.getEntriesByType('resource')
    .find(entry => new URL(entry.name).pathname.endsWith(suffix))?.name;
  const api = await import(moduleUrl('/core/api.js'));
  const preview = await import(moduleUrl('/ui/preview.js'));
  let saved = await api.call('get_settings');
  check(!('cookieText' in saved) && !('cookieDomain' in saved) && !('cookieSite' in saved), 'Import-only fields are not persisted');
  click('[data-action="open-settings"]'); await wait();
  click('[data-action="settings-tab"][data-tab="network"]'); await wait();
  check(document.querySelector('[data-setting="cookieText"]').value === '', 'Custom Cookie cleared after save');
  set('cookiesFile', '');
  set('cookiesBrowser', 'chrome');
  click('[data-action="save-settings"]'); await wait(); await wait();
  const showBrowserError = async () => {
    saved = await api.call('get_settings');
    preview.patchPreview({
      probe: { ok: false, error: 'Could not copy Chrome cookie database', log: [] },
      settings: saved, parserOpen: true, analyzing: false, language: 'zh', parserOptions: {},
    });
  };
  input.value = 'https://www.youtube.com/watch?v=BaW_jenozKc';
  input.dispatchEvent(new Event('input', {bubbles:true}));
  click('[data-action="analyze"]'); await wait(); await wait();
  await showBrowserError();
  click('[data-action="open-cookie-settings"]'); await wait();
  check(document.querySelector('[data-setting="cookieSite"]').value === 'youtube', 'YouTube error opens the matching Cookie site');
  click('[data-action="close-settings"]'); await wait();
  click('[data-action="analyze"]'); await wait(); await wait();
  await showBrowserError();
  click('[data-action="disable-browser-cookies-and-retry"]'); await wait(); await wait();
  saved = await api.call('get_settings');
  check(saved.cookiesBrowser === '', 'Retry persistently disables browser extraction');
  check(document.querySelector('.parser-dialog') && !document.querySelector('[data-action="disable-browser-cookies-and-retry"]'), 'Retry completes without the browser error');
  input.value = 'https://video.example.com/watch/123';
  input.dispatchEvent(new Event('input', {bubbles:true}));
  click('[data-action="analyze"]'); await wait(); await wait();
  await showBrowserError();
  click('[data-action="open-cookie-settings"]'); await wait();
  check(document.querySelector('[data-setting="cookieSite"]').value === 'custom', 'Unknown website uses explicit custom source');
  check(document.querySelector('[data-setting="cookieDomain"]').value === 'video.example.com', 'Custom source keeps the exact URL hostname');
  return {qualityExplanation:true,networkShortcut:true,cookieImport:true,douyinCookieImport:true,youtubeCookieImport:true,customCookieImport:true,rawCookieCleared:true,sourceSelection:true,browserRetry:true};
})()
