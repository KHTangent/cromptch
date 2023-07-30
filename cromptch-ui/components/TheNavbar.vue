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
	</v-app-bar>
</template>

<script lang="ts" setup>
import { mdiGlobeModel } from "@mdi/js";

const navIcon = mdiGlobeModel;
</script>
