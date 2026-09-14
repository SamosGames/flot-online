// SPDX-FileCopyrightText: 2026 SamosGames
// SPDX-License-Identifier: AGPL-3.0-or-later

(() => {
    const host = window.location.hostname;
    const params = new URLSearchParams(window.location.search);
    const isYandex = host.includes("yandex") || params.has("yandex");

    // kodiak discovers the websocket host from the response URL of /system.json.
    // A Yandex package is static, so a staging origin can be supplied explicitly.
    const serverOrigin = window.__MK48_SERVER_ORIGIN || params.get("server_origin");
    if (serverOrigin) {
        try {
            const origin = new URL(serverOrigin, window.location.href).origin;
            const nativeFetch = window.fetch.bind(window);
            window.fetch = (input, init) => {
                const requested = typeof input === "string" ? input : input?.url;
                if (!requested) return nativeFetch(input, init);
                const url = new URL(requested, window.location.href);
                if (url.pathname !== "/system.json" && url.pathname !== "/translation.json") {
                    return nativeFetch(input, init);
                }
                return nativeFetch(`${origin}${url.pathname}${url.search}`, init);
            };
        } catch (error) {
            console.warn("Invalid MK48 server origin:", error);
        }
    }

    if (!isYandex) return;

    let sdkPromise;
    let adBusy = false;

    const notify = (message) => window.postMessage(message, "*");

    function loadSdk() {
        if (sdkPromise) return sdkPromise;

        sdkPromise = new Promise((resolve, reject) => {
            if (window.YaGames) {
                resolve(window.YaGames);
                return;
            }

            const script = document.createElement("script");
            script.src = "https://yandex.ru/games/sdk/v2";
            script.async = true;
            script.onload = () => window.YaGames ? resolve(window.YaGames) : reject(new Error("YaGames is unavailable"));
            script.onerror = () => reject(new Error("Yandex Games SDK failed to load"));
            document.head.appendChild(script);
        }).then((YaGames) => YaGames.init());

        sdkPromise.catch((error) => console.warn("Yandex Games SDK:", error));
        return sdkPromise;
    }

    function enableAds() {
        loadSdk().then((ysdk) => {
            window.__mk48YandexSdk = ysdk;
            const language = ysdk.environment?.i18n?.lang;
            if (language) document.documentElement.lang = language;
            notify("enableInterstitialAds");
            notify("enableRewardedAds");
            notify("enableBannerAds");
        }).catch(() => {});
    }

    function showFullscreen() {
        if (adBusy) return;
        adBusy = true;
        loadSdk().then((ysdk) => {
            ysdk.adv.showFullscreenAdv({
                callbacks: {
                    onOpen: () => {},
                    onClose: () => {
                        adBusy = false;
                        notify("tallyInterstitialAd");
                    },
                    onError: () => {
                        adBusy = false;
                        notify("cancelInterstitialAd");
                    },
                    onOffline: () => {
                        adBusy = false;
                        notify("cancelInterstitialAd");
                    },
                },
            });
        }).catch(() => {
            adBusy = false;
            notify("cancelInterstitialAd");
        });
    }

    function showRewarded() {
        if (adBusy) return;
        adBusy = true;
        let rewarded = false;
        loadSdk().then((ysdk) => {
            ysdk.adv.showRewardedVideo({
                callbacks: {
                    onOpen: () => {},
                    onRewarded: () => { rewarded = true; },
                    onClose: () => {
                        adBusy = false;
                        notify(rewarded ? "tallyRewardedAd" : "cancelRewardedAd");
                    },
                    onError: () => {
                        adBusy = false;
                        notify("cancelRewardedAd");
                    },
                },
            });
        }).catch(() => {
            adBusy = false;
            notify("cancelRewardedAd");
        });
    }

    function showBanner() {
        loadSdk().then((ysdk) => ysdk.adv.showBannerAdv()).then(() => notify("tallyBannerAd")).catch(() => {});
    }

    function hideBanner() {
        loadSdk().then((ysdk) => ysdk.adv.hideBannerAdv()).catch(() => {});
    }

    window.addEventListener("message", (event) => {
        switch (event.data) {
            case "requestInterstitialAd": showFullscreen(); break;
            case "requestRewardedAd": showRewarded(); break;
            case "requestBannerAd": showBanner(); break;
            case "hideBannerAd": hideBanner(); break;
            case "gameLoaded": enableAds(); break;
            default: break;
        }
    });

    enableAds();
})();
