<template>
	<v-card class="flex-grow-1 d-flex flex-column">
		<v-card-title class="text-h3 my-3">
			{{ recipe.title }}
		</v-card-title>
		<v-card-text class="mt-0 pt-0 flex-grow-1 d-flex flex-column">
			<NuxtLink :to="'/recipe/' + recipe.id" class="pa-1">
				<v-img
					contain
					:src="imageSrc()"
					aspect-ratio="1"
					height="300"
					alt="Placeholder image"
				/>
			</NuxtLink>
			<span class="text-body-1">
				{{ recipe.description.substring(0, 100) }}
			</span>
		</v-card-text>
	</v-card>
</template>

<script setup lang="ts">
import { type PropType } from "vue";
import * as Api from "@/scripts/api";
import * as APITypes from "@/scripts/apiTypes";

const props = defineProps({
	recipe: {
		type: Object as PropType<APITypes.RecipeMetadata>,
		required: true,
	},
});

function imageSrc() {
	if (props.recipe.imageId) {
		return Api.getImageThumbnailUrl(props.recipe.imageId!);
	}
	return "";
}
</script>
