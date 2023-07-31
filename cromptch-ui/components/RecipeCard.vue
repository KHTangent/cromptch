<template>
	<v-card class="flex-grow-1 d-flex flex-column">
		<v-card-title class="text-h2 mb-0 pb-0">
			{{ recipe.title }}
		</v-card-title>
		<v-card-text class="mt-0 pt-0 flex-grow-1 d-flex flex-column">
			<a href="/" class="pa-1">
				<v-img
					contain
					src="previewLink"
					aspect-ratio="1"
					height="300"
					alt="selectedFilename"
				/>
			</a>
			<span class="text-body-1">
				{{ recipe.description.substring(0, 100) }}
			</span>
		</v-card-text>
	</v-card>
</template>

<script setup lang="ts">
import { PropType } from "vue";
import * as APITypes from "@/scripts/apiTypes";

defineProps({
	recipe: {
		type: Object as PropType<APITypes.RecipeMetadata>,
		required: true,
	},
});
</script>
