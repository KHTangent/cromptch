<template>
	<v-container>
		<h1 class="text-h1 mb-4">Sign up</h1>
		<v-alert type="error" v-if="error.length > 0" dismissible class="ma-2">
			{{ error }}
		</v-alert>
		<v-alert type="success" v-if="registrationSuccess" dismissible class="ma-2">
			Registration successful! You can now
			<nuxt-link to="/login">log in</nuxt-link>.
		</v-alert>
		<v-form @submit.prevent="register" class="ma-2">
			<v-text-field label="Username" v-model="username" required></v-text-field>
			<v-text-field label="Email" v-model="email" required></v-text-field>
			<v-text-field
				label="Password"
				type="password"
				v-model="password"
				required
			></v-text-field>
			<v-text-field
				label="Confirm Password"
				type="password"
				v-model="confirmPassword"
				required
			></v-text-field>
			<vue-hcaptcha
				v-if="hcaptchaEnabled"
				:sitekey="hcaptchaSiteKey"
				@verify="onHCVerify"
				@expired="onHCExpired"
			></vue-hcaptcha>
			<v-btn color="primary" class="mt-4" type="submit">Sign up</v-btn>
		</v-form>
	</v-container>
</template>
<script lang="ts" setup>
import * as Api from "@/scripts/api";
import VueHcaptcha from "@hcaptcha/vue3-hcaptcha";

const appConfig = useRuntimeConfig();

const hcaptchaSiteKey = computed(() => appConfig.public.hcaptchaSiteKey as string);
const hcaptchaEnabled = computed(() => hcaptchaSiteKey.value.length > 0);

const username = ref("");
const email = ref("");
const password = ref("");
const confirmPassword = ref("");
const hcaptchaToken = ref("");

const error = ref("");
const registrationSuccess = ref(false);

function onHCVerify(token: string, e: any) {
	hcaptchaToken.value = token;
}

function onHCExpired() {
	hcaptchaToken.value = "";
}

async function register() {
	if (password.value !== confirmPassword.value) {
		error.value = "Passwords do not match";
		return;
	}
	if (password.value.length < 8) {
		error.value = "Password must be at least 8 characters";
		return;
	}
	if (username.value.length < 3) {
		error.value = "Username must be at least 3 characters";
		return;
	}
	if (email.value.length < 3 || !email.value.includes("@")) {
		error.value = "Invalid email address";
		return;
	}
	try {
		await Api.register(
			username.value,
			email.value,
			password.value,
			hcaptchaEnabled ? hcaptchaToken.value : undefined,
		);
		error.value = "";
		registrationSuccess.value = true;
	} catch (e: any) {
		error.value = e.data;
	}
}
</script>
