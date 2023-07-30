import { createVuetify } from "vuetify";
import { aliases, mdi } from "vuetify/iconsets/mdi-svg";

export default defineNuxtPlugin((nuxtApp) => {
	const vuetify = createVuetify({
		ssr: process.server,
		theme: {
			defaultTheme: "dark",
		},
		icons: {
			defaultSet: "mdi",
			aliases,
			sets: { mdi },
		},
	});
	nuxtApp.vueApp.use(vuetify);
});
