<template>
	<v-container>
		<v-container v-if="getHeaderImageUrl().length > 0">
			<v-img
				cover
				max-height="500"
				:src="getHeaderImageUrl()"
			></v-img>
		</v-container>
		<h1 :class="`text-h${isMobile ? '2' : '1'} mb-4`">{{ recipe.metadata.title }}</h1>
		<p class="text-body-1">{{ recipe.metadata.description }}</p>
		<p class="text-body-1">
			Uploaded by <strong>{{ author }}</strong>
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
								{{ scaleIngredient(ingredient.quantity) }}
							</v-col>
							<v-col cols="2">
								{{ ingredient.unit }}
							</v-col>
							<v-col cols="7">
								{{ ingredient.name }}
							</v-col>
						</v-row>
					</v-list-item>
					<v-list-item>
						<v-row align="center">
							<v-col cols="4">
								<strong>Scale: </strong>
							</v-col>
							<v-col cols="3">
								<v-btn icon variant="text" @click="decreaseScale">
									<v-icon>{{ icons.mdiMinus }}</v-icon>
								</v-btn>
							</v-col>
							<v-col cols="2">
								{{ ingredientScaleFactor }}
							</v-col>
							<v-col cols="3">
								<v-btn icon variant="text" @click="increaseScale">
									<v-icon>{{ icons.mdiPlus }}</v-icon>
								</v-btn>
							</v-col>
						</v-row>
					</v-list-item>
				</v-list>
			</v-col>
			<v-col cols="12" md="6" lg="8">
				<h2 class="text-h2">Steps</h2>
				<v-list class="my-2">
					<v-list-item v-for="(step, i) in recipe.steps">
						<p class="text-body-1 ma-3" :key="i">
							<strong>Step {{ i + 1 }}:</strong>
							{{ step.description }}
						</p>
						<v-img
							v-if="step.imageId"
							:src="getStepImageUrl(step.imageId)"
							max-height="500"
							class="mb-4 mt-n2"
						></v-img>
					</v-list-item>
					</v-list>
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
const isMobile = useDisplay().mobile;
const { recipe, author } = await API.getRecipe(route.params.id as string);
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

function getHeaderImageUrl(): string {
	if (!recipe.metadata.imageId) {
		return "";
	}
	return API.getImageUrl(recipe.metadata.imageId);
}

function getStepImageUrl(id: string): string {
	return API.getImageUrl(id);
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
