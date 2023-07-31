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
			<v-btn color="primary" class="mt-4" type="submit">Sign up</v-btn>
		</v-form>
	</v-container>
</template>
<script lang="ts" setup>
import * as Api from "@/scripts/api";

const username = ref("");
const email = ref("");
const password = ref("");
const confirmPassword = ref("");

const error = ref("");
const registrationSuccess = ref(false);

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
	const res = await Api.register(username.value, email.value, password.value);
	if (res.status === 200) {
		error.value = "";
		registrationSuccess.value = true;
	} else {
		error.value = await res.text();
	}
}
</script>
