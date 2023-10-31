<template>
	<v-container>
		<h1 class="text-h1 mb-4">{{ recipe.title }}</h1>
		<p class="text-body-1">{{ recipe.description }}</p>
		<p class="text-body-1">
			Uploaded by <strong>{{ recipe.author }}</strong>
		</p>
		<v-divider class="my-2"></v-divider>
		<v-row>
			<v-col cols="12" md="6" lg="4">
				<v-list>
					<v-list-item type="subheader"> Ingredients </v-list-item>
					<v-list-item>
						<v-row>
							<v-col cols="3">
								<strong>Amount</strong>
							</v-col>
							<v-col cols="2">
								<strong>Unit</strong>
							</v-col>
							<v-col cols="7">
								<strong>Ingredient</strong>
							</v-col>
						</v-row>
					</v-list-item>
					<v-list-item v-for="(ingredient, i) in recipe.ingredients" :key="i">
						<v-row>
							<v-col cols="3">
								{{ scaleIngredient(ingredient[0]) }}
							</v-col>
							<v-col cols="2">
								{{ ingredient[1] }}
							</v-col>
							<v-col cols="7">
								{{ ingredient[2] }}
							</v-col>
						</v-row>
					</v-list-item>
					<v-list-item>
						<v-row align="center">
							<v-col cols="6">
								<strong>Scale: </strong>
							</v-col>
							<v-col cols="2">
								<v-btn icon color="blue-darken-4" @click="decreaseScale">
									<v-icon>{{ icons.mdiMinus }}</v-icon>
								</v-btn>
							</v-col>
							<v-col cols="2">
								{{ ingredientScaleFactor }}
							</v-col>
							<v-col cols="2">
								<v-btn icon color="blue-darken-2" @click="increaseScale">
									<v-icon>{{ icons.mdiPlus }}</v-icon>
								</v-btn>
							</v-col>
						</v-row>
					</v-list-item>
				</v-list>
			</v-col>
			<v-col cols="12" md="6" lg="8">
				<h2 class="text-h2">Steps</h2>
				<p class="text-body-1 ma-3" v-for="(step, i) in recipe.steps" :key="i">
					<strong>Step {{ i + 1 }}:</strong>
					{{ step }}
				</p>
			</v-col>
		</v-row>
	</v-container>
</template>
<script setup lang="ts">
import * as API from "@/scripts/api";
import { mdiMinus, mdiPlus } from "@mdi/js";

const icons = {
	mdiPlus,
	mdiMinus,
};

const route = useRoute();
const recipe = await API.getRecipe(route.params.id as string);
const ingredientScaleFactor = ref(1);

function increaseScale() {
	if (ingredientScaleFactor.value < 2) {
		ingredientScaleFactor.value += 0.25;
	} else if (ingredientScaleFactor.value < 5) {
		ingredientScaleFactor.value += 0.5;
	} else {
		ingredientScaleFactor.value += 1;
	}
}

function decreaseScale() {
	if (ingredientScaleFactor.value <= 0.25) {
		ingredientScaleFactor.value = 0.25;
	} else if (ingredientScaleFactor.value < 2) {
		ingredientScaleFactor.value -= 0.25;
	} else if (ingredientScaleFactor.value < 5) {
		ingredientScaleFactor.value -= 0.5;
	} else {
		ingredientScaleFactor.value -= 1;
	}
}

function scaleIngredient(ingredient: number): number {
	return parseFloat((ingredient * ingredientScaleFactor.value).toPrecision(3));
}
</script>
