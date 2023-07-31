<template>
	<v-container>
		<h1 class="text-h1 mb-4">{{ recipe.title }}</h1>
		<p class="text-body-1">{{ recipe.description }}</p>
		<v-divider class="my-2"></v-divider>
		<v-row>
			<v-col cols="12" md="4">
				<v-list>
					<v-list-item type="subheader"> Ingredients </v-list-item>
					<v-list-item v-for="(ingredient, i) in recipe.ingredients" :key="i">
						<v-row>
							<v-col cols="2">
								{{ ingredient[0] }}
							</v-col>
							<v-col cols="2">
								{{ ingredient[1] }}
							</v-col>
							<v-col cols="8">
								{{ ingredient[2] }}
							</v-col>
						</v-row>
					</v-list-item>
				</v-list>
			</v-col>
			<v-col cols="12" md="8">
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
const route = useRoute();
const recipe = await API.getRecipe(route.params.id as string);
</script>
