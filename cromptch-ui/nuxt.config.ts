import { defineNuxtConfig } from 'nuxt/config'

export default defineNuxtConfig({
	devtools: { enabled: true },
	modules: ["vuetify-nuxt-module"],
	runtimeConfig: {
		public: {
			hcaptchaSiteKey: "",
			apiUrl: "",
		}
	},
	vuetify: {
		vuetifyOptions: {
			theme: {
				defaultTheme: "dark",
			},
			icons: {
				defaultSet: "mdi-svg"
			},
		}
	}
});
