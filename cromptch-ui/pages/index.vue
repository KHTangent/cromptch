<template>
	<v-container>
		<h1 class="text-h1 mb-4">Hello world!</h1>
		<p class="text-body-1">Welcome to this template</p>
		<v-row>
			<v-col v-for="recipe in recipes" :key="recipe.id" cols="12" md="6" lg="4">
				<RecipeCard :recipe="recipe" />
			</v-col>
		</v-row>
	</v-container>
</template>
<script setup lang="ts">
import * as API from "@/scripts/api";

const recipes = await API.getRecipeList();
</script>
