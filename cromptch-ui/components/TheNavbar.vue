<template>
	<v-app-bar>
		<NuxtLink to="/" class="mx-2 my-n1">
			<v-avatar>
				<v-img
					src="/icon/pizza256.png"
					alt="Cromptch icon"
				></v-img>
			</v-avatar>
		</NuxtLink>
		<v-tabs v-if="!isMobile">
			<v-tab to="/"> Home </v-tab>
			<v-tab v-if="loggedIn" to="/recipe/create"> Create recipe </v-tab>
			<v-tab v-if="isAdmin" to="/admin"> Admin </v-tab>
		</v-tabs>

		<v-spacer></v-spacer>

		<span v-if="!isMobile" class="mr-4">
			<span v-if="loggedIn">
				Logged in as {{ profile.data.value?.username }}
			</span>
			<span v-else>
				<v-btn variant="text" to="/login"> Login </v-btn>
				<v-btn variant="text" to="/register"> Register </v-btn>
			</span>
		</span>
		<v-btn
			icon
			v-if="isMobile"
			@click="sideMenuOpen = !sideMenuOpen"
		>
			<v-icon>
				{{ sideMenuOpen ? icons.mdiClose : icons.mdiMenu }}
			</v-icon>
		</v-btn>
		<Teleport to="body">
			<v-navigation-drawer
				temporary
				location="right"
				v-model="sideMenuOpen"
			>
				<v-list nav>
					<v-list-item to="/"> Home </v-list-item>
					<v-list-item v-if="loggedIn" to="/recipe/create"> Create recipe </v-list-item>
					<v-list-item v-if="isAdmin" to="/admin"> Admin </v-list-item>
				</v-list>

				<template #append>
					<div v-if="loggedIn" class="pa-2">
						Logged in as {{ profile.data.value?.username }}
					</div>
					<div v-else class="pa-2">
						<v-btn class="ma-1" block to="/login"> Login </v-btn>
						<v-btn class="ma-1" block to="/register"> Register </v-btn>
					</div>
				</template>
			</v-navigation-drawer>
		</Teleport>
	</v-app-bar>
</template>

<script lang="ts" setup>
import { mdiMenu, mdiClose } from "@mdi/js";

const icons = { mdiMenu, mdiClose };

const profile = await useLocalUser();
const loggedIn = computed(() => !!profile.data.value);
const isAdmin = computed(() => profile.data.value && profile.data.value.isAdmin);
const isMobile = useDisplay().mobile;

const sideMenuOpen = ref(false);
</script>
