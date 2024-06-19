<template>
	<v-container>
		<h1 :class="`text-h${isMobile ? '2' : '1'} mb-4`">Cromptch: Share recipes</h1>
		<v-container class="d-flex flex-direction-row justify-end">
			<v-btn-toggle v-model="selectedSortMode">
				<v-btn icon>
					<v-icon>{{ icons.mdiSortAlphabeticalAscending }}</v-icon>
				</v-btn>
				<v-btn icon>
					<v-icon>{{ icons.mdiSortAlphabeticalDescending }}</v-icon>
				</v-btn>
				<v-btn icon>
					<v-icon>{{ icons.mdiSortCalendarAscending }}</v-icon>
				</v-btn>
				<v-btn icon>
					<v-icon>{{ icons.mdiSortCalendarDescending }}</v-icon>
				</v-btn>
			</v-btn-toggle>
		</v-container>
		<v-row v-if="recipes.length > 0">
			<v-col v-for="recipe in recipes" :key="recipe.id" cols="12" md="6" lg="4">
				<RecipeCard :recipe="recipe" />
			</v-col>
		</v-row>
	</v-container>
</template>
<script setup lang="ts">
import * as APITypes from "@/scripts/apiTypes";
import { mdiSortAlphabeticalAscending, mdiSortAlphabeticalDescending, mdiSortCalendarAscending, mdiSortCalendarDescending } from "@mdi/js";

const icons = {
	mdiSortAlphabeticalAscending,
	mdiSortAlphabeticalDescending,
	mdiSortCalendarAscending,
	mdiSortCalendarDescending,
};
const sortOrders = [
	APITypes.RecipeListSortTypes.NameAscending,
	APITypes.RecipeListSortTypes.NameDescending,
	APITypes.RecipeListSortTypes.DateAscending,
	APITypes.RecipeListSortTypes.DateDescending,
];

const isMobile = useDisplay().mobile;

let selectedSortMode = ref(0);

let recipes: Ref<APITypes.RecipeMetadata[]> = ref([]);

watch(selectedSortMode, async (newValue, oldValue) => {
	if (newValue == oldValue) return;
	try {
		const newRecipes = await useBackend().getRecipeList(sortOrders[newValue]);
		recipes.value = newRecipes;
	} catch (e: any) {}
});

try {
	recipes.value = await useBackend().getRecipeList(APITypes.RecipeListSortTypes.DateDescending);
} catch (e: any) {}
</script>
