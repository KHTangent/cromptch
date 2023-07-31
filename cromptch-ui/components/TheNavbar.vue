<template>
	<v-app-bar>
		<v-app-bar-nav-icon>
			<v-icon>
				{{ navIcon }}
			</v-icon>
		</v-app-bar-nav-icon>
		<v-tabs>
			<v-tab to="/"> Home </v-tab>
			<v-tab to="/about"> About </v-tab>
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
