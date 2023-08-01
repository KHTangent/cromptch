// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
	devtools: { enabled: true },
	modules: ["./modules/vuetify.ts"],
	runtimeConfig: {
		public: {
			hcaptchaSiteKey: "aaa",
		}
	}
});
