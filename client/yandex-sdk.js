// SPDX-FileCopyrightText: 2026 SamosGames
// SPDX-License-Identifier: AGPL-3.0-or-later

(() => {
    const host = window.location.hostname;
    const params = new URLSearchParams(window.location.search);
    const isYandex = host.includes("yandex") || params.has("yandex");
    const isRussianProduct = true;

    // kodiak discovers the websocket host from the response URL of /system.json.
    // A Yandex package is static, so a staging origin can be supplied explicitly.
    const serverOrigin = window.__FLOT_ONLINE_SERVER_ORIGIN || params.get("server_origin");
    const nativeFetch = window.fetch.bind(window);
    let serverUrl;
    if (serverOrigin) {
        try {
            serverUrl = new URL(serverOrigin, window.location.href).origin;
        } catch (error) {
            console.warn("Invalid SamosGames server origin:", error);
        }
    }

    // Keep the Russian package self-contained: Yandex does not need the
    // infrastructure translation service just to render the first screen.
    window.fetch = (input, init) => {
        const requested = typeof input === "string" ? input : input?.url;
        if (!requested) return nativeFetch(input, init);
        const url = new URL(requested, window.location.href);
        if (url.pathname === "/translation.json" && isRussianProduct) {
            return nativeFetch("/data/translation.ru.json", init);
        }
        if (serverUrl && url.pathname === "/system.json") {
            return nativeFetch(`${serverUrl}${url.pathname}${url.search}`, init);
        }
        if (serverUrl && url.pathname === "/translation.json") {
            return nativeFetch(`${serverUrl}${url.pathname}${url.search}`, init);
        }
        return nativeFetch(input, init);
    };

    function installRussianUiBridge() {
        if (window.__flotOnlineRussianUiObserver) return;

        const replacements = new Map([
            ["Ad Privacy", "Настройки рекламы"],
            ["By changing settings, you consent to cookies being stored in accordance with our ", "Изменяя настройки, вы соглашаетесь на сохранение cookie в соответствии с нашей "],
            ["privacy policy", "политикой конфиденциальности"],
            ["Loading regions...", "Загрузка регионов..."],
            ["-- Other --", "-- Другое --"],
            ["Unknown server", "Неизвестный сервер"],
            ["Refresh", "Обновить"],
            ["Necessary cookies only", "Только необходимые cookie"],
            ["Allow all cookies", "Разрешить все cookie"],
            ["Automated help: ", "Автопомощь: "],
            ["Local", "Локальный"],
        ]);

        const translateNode = (node) => {
            if (node.nodeType === Node.TEXT_NODE) {
                let text = node.nodeValue;
                for (const [from, to] of replacements) text = text.replaceAll(from, to);
                if (text !== node.nodeValue) node.nodeValue = text;
                return;
            }
            for (const child of node.childNodes || []) translateNode(child);
        };

        translateNode(document.documentElement);
        window.__flotOnlineRussianUiObserver = new MutationObserver((mutations) => {
            for (const mutation of mutations) {
                if (mutation.type === "characterData") translateNode(mutation.target);
                for (const node of mutation.addedNodes || []) translateNode(node);
            }
        });
        window.__flotOnlineRussianUiObserver.observe(document.documentElement, {
            childList: true,
            characterData: true,
            subtree: true,
        });
    }

    if (isRussianProduct) {
        // Kodiak uses navigator.language for the first saved preference.
        // The language picker can still change it afterwards.
        try {
            Object.defineProperty(window.navigator, "language", {
                configurable: true,
                get: () => "ru-RU",
            });
            Object.defineProperty(window.navigator, "languages", {
                configurable: true,
                get: () => ["ru-RU", "ru"],
            });
            document.documentElement.lang = "ru";
        } catch (error) {
            console.warn("Unable to set Russian default locale:", error);
        }
        installRussianUiBridge();
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
            window.__flotOnlineYandexSdk = ysdk;
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
