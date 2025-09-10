const CACHE_NAME = 'image-resizer-cache-v1';
const urlsToCache = [
  './',
  './index.html',
  './logo.png',
  './pkg/rust_image.js',
  './pkg/rust_image_bg.wasm',
  './alpine.js',
  './offline.html'
];

// インストール時のキャッシュ
self.addEventListener('install', event => {
  event.waitUntil(
    caches.open(CACHE_NAME)
      .then(cache => cache.addAll(urlsToCache))
  );
});

// アクティベート時の古いキャッシュの削除
self.addEventListener('activate', event => {
  event.waitUntil(
    caches.keys().then(cacheNames => {
      return Promise.all(
        cacheNames.map(cacheName => {
          if (cacheName !== CACHE_NAME) {
            return caches.delete(cacheName);
          }
        })
      );
    })
  );
});

// フェッチ時のキャッシュ更新戦略
self.addEventListener('fetch', event => {
  event.respondWith(
    caches.match(event.request)
      .then(response => {
        // キャッシュがあればそれを返す
        if (response) {
          // バックグラウンドでキャッシュを更新
          fetch(event.request).then(response => {
            if (response && response.status === 200) {
              caches.open(CACHE_NAME).then(cache => {
                cache.put(event.request, response);
              });
            }
          }).catch(() => {/* エラーは無視 */});
          return response;
        }

        // キャッシュがない場合はネットワークリクエスト
        return fetch(event.request).then(response => {
          // レスポンスが有効な場合のみキャッシュ
          if (!response || response.status !== 200) {
            return response;
          }

          // レスポンスを複製（ストリームは一度しか読めないため）
          const responseToCache = response.clone();
          caches.open(CACHE_NAME).then(cache => {
            cache.put(event.request, responseToCache);
          });

          return response;
        }).catch(() => caches.match('./offline.html'));
      })
  );
});
