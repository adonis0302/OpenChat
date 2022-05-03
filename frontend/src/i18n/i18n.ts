import { init, locale, addMessages, getLocaleFromNavigator } from "svelte-i18n";

import en from "./en.json";
import fr from "./fr.json";
import de from "./de.json";
import es from "./es.json";
import vi from "./vi.json";
import it from "./it.json";
import cn from "./cn.json";
import jp from "./jp.json";
import ru from "./ru.json";
import iw from "./iw.json";

export const translationCodes: Record<string, string> = {
    cn: "zh-cn",
    de: "de",
    es: "es",
    en: "en",
    fr: "fr",
    it: "it",
    jp: "ja",
    ru: "ru",
    vi: "vi",
    iw: "iw",
};

export const supportedLanguages = [
    {
        name: "English",
        code: "en",
        json: en,
    },
    {
        name: "Français",
        code: "fr",
        json: fr,
    },
    {
        name: "Deutsch",
        code: "de",
        json: de,
    },
    {
        name: "Italiano",
        code: "it",
        json: it,
    },
    {
        name: "Español",
        code: "es",
        json: es,
    },
    {
        name: "Tiếng Việt",
        code: "vi",
        json: vi,
    },
    {
        name: "中文",
        code: "cn",
        json: cn,
    },
    {
        name: "日本",
        code: "jp",
        json: jp,
    },
    {
        name: "русский",
        code: "ru",
        json: ru,
    },
    {
        name: "עִברִית",
        code: "iw",
        json: iw,
    },
];

supportedLanguages.forEach(({ code, json }) => {
    addMessages(code, json);
});

init({
    fallbackLocale: "en",
    initialLocale: getStoredLocale(),
});

export function getStoredLocale(): string {
    return localStorage.getItem("openchat_locale") ?? (getLocaleFromNavigator() || "en");
}

export function setLocale(code: string): void {
    locale.set(code);
    localStorage.setItem("openchat_locale", code);
}
