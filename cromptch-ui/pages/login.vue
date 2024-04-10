<template>
	<v-container>
		<h1 :class="`text-h${isMobile ? '2' : '1'} mb-4`">Log in</h1>
		<v-alert type="error" v-if="error.length > 0" dismissible class="ma-2">
			{{ error }}
		</v-alert>
		<v-form @submit.prevent="login" class="ma-2">
			<v-text-field label="Email" v-model="email" required></v-text-field>
			<v-text-field
				label="Password"
				type="password"
				v-model="password"
				required
			></v-text-field>
			<v-btn color="primary" class="mt-4" type="submit">Log in</v-btn>
		</v-form>
	</v-container>
</template>
<script lang="ts" setup>
import * as Api from "@/scripts/api";
import * as ApiTypes from "@/scripts/apiTypes";

const isMobile = useDisplay().mobile;

const email = ref("");
const password = ref("");

const error = ref("");

async function login() {
	if (email.value.length === 0 || password.value.length === 0) {
		return;
	}
	let user: ApiTypes.LoginResponse;
	try {
		user = await Api.login(email.value, password.value);
	} catch (e: unknown) {
		if (e instanceof Response) {
			error.value = await e.text();
		}
		return;
	}
	const cookieToken = useCookie("token");
	cookieToken.value = user.token;
	let localToken = useToken();
	localToken.value = user.token;
	navigateTo("/");
}
</script>
