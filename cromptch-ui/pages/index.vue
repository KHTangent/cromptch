<template>
	<v-container>
		<h1 class="text-h1 mb-4">Cromptch: Share recipes</h1>
		<v-row v-if="recipes.length > 0">
			<v-col v-for="recipe in recipes" :key="recipe.id" cols="12" md="6" lg="4">
				<RecipeCard :recipe="recipe" />
			</v-col>
		</v-row>
	</v-container>
</template>
<script setup lang="ts">
import * as API from "@/scripts/api";
import * as APITypes from "@/scripts/apiTypes";
let recipes: APITypes.RecipeMetadata[] = [];
try {
	recipes = await API.getRecipeList();
} catch (e: any) {}
</script>
