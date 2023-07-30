<template>
	<v-app>
		<TheNavbar />
		<v-main class="my-4">
			<slot></slot>
		</v-main>
		<TheFooter />
	</v-app>
</template>

<script lang="ts" setup></script>
