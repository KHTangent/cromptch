<template>
	<v-app-bar class="pr-4">
		<NuxtLink to="/" class="mx-2 my-n1">
			<v-avatar>
				<v-img
					src="/icon/pizza256.png"
					alt="Cromptch icon"
				></v-img>
			</v-avatar>
		</NuxtLink>
		<v-tabs>
			<v-tab to="/"> Home </v-tab>
			<v-tab v-if="loggedIn" to="/recipe/create"> Create recipe </v-tab>
		</v-tabs>
		<v-spacer></v-spacer>

		<span v-if="loggedIn">
			Logged in as {{ profile.data.value?.username }}
		</span>
		<span v-else>
			<v-btn variant="text" to="/login"> Login </v-btn>
			<v-btn variant="text" to="/register"> Register </v-btn>
		</span>
	</v-app-bar>
</template>

<script lang="ts" setup>
import { mdiGlobeModel } from "@mdi/js";

const navIcon = mdiGlobeModel;

const profile = await useLocalUser();
const loggedIn = computed(() => !!profile.data.value);
</script>
