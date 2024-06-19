<template>
	<v-container>
		<h1 :class="`text-h${isMobile ? '2' : '1'} mb-4`">Create recipe</h1>
		<v-alert type="error" v-if="error.length > 0" dismissible class="ma-2">
			{{ error }}
		</v-alert>
		<p class="text-body-1">
			A recipe must have at least one step and one ingredient.
		</p>
		<v-divider class="my-2"></v-divider>
		<RecipeEditor
			v-model="recipe"
			@submit="submitAndRedirect"
		></RecipeEditor>
	</v-container>
</template>
<script setup lang="ts">
import { type CreateRecipeRequest, type CreateRecipeResponse } from "@/scripts/apiTypes";
import { FetchError } from "ofetch";

const recipe = ref<CreateRecipeRequest>({
	description: "",
	name: "",
	ingredients: [{
		quantity: 0.0,
		unit: "",
		name: "",
	}],
	steps: [{
		description: "",
		imageId: "",
	}],
});
const error = ref("");

const userToken = useToken();
const isMobile = useDisplay().mobile;


async function submitAndRedirect() {
	if (recipe.value.imageId?.length == 0) {
		recipe.value.imageId = undefined;
	}
	for (let step of recipe.value.steps) {
		if (step.imageId?.length == 0) {
			step.imageId = undefined;
		}
	}
	let response: CreateRecipeResponse;
	try {
		response = await useBackend().createRecipe(recipe.value, userToken.value);
	} catch (e: unknown) {
		if (e instanceof FetchError) {
			error.value = await e.data;
		}
		return;
	}
	navigateTo(`/recipe/${response.id}`);
}
</script>
