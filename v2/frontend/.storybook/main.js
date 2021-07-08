module.exports = {
    stories: ["../src/**/*.stories.mdx", "../src/**/*.stories.@(js|jsx|ts|tsx|svelte)"],
    addons: [
        "@storybook/addon-links",
        "@storybook/addon-essentials",
        "@storybook/addon-svelte-csf",
        "@storybook/addon-a11y",
        "storybook-addon-locale",
    ],
    svelteOptions: {
        preprocess: require("svelte-preprocess")({
            scss: {
                prependData: `@import 'src/styles/mixins.scss';`,
            },
        }),
    },
};
