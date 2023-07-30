import type { NuxtModule } from "@nuxt/schema";
import vuetifyLoader from "vite-plugin-vuetify";

// eslint-disable-next-line @typescript-eslint/require-await
const vuetifyModule: NuxtModule = async (_inlineOptions, nuxt) => {
	nuxt.options.build.transpile.push("vuetify");

	nuxt.options.css.push("vuetify/lib/styles/main.css");

	nuxt.options.vite.ssr ??= {};
	nuxt.options.vite.ssr.noExternal ??= [];
	if (!Array.isArray(nuxt.options.vite.ssr.noExternal)) {
		throw new Error(
			"Expected nuxt.options.vite.ssr.noExternal to be an array.",
		);
	}
	nuxt.options.vite.ssr.noExternal.push("vuetify");

	nuxt.hooks.hook("vite:extendConfig", (config) => {
		config.plugins?.push(vuetifyLoader());
	});
};

export default vuetifyModule;
