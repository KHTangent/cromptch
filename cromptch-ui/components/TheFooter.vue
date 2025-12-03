<template>
	<v-footer>
		<v-container
			:class="`d-flex ${isMobile ? 'flex-column' : 'flex-row'} align-center justify-space-between`"
		>
			<span class="text-body-1">
				&copy; {{ new Date().getFullYear() }} - KHTangent, icon by SassyBuns
			</span>
			<span v-if="isMobile" class="my-2"></span>
			<v-btn
				variant="text"
				target="_blank"
				:block="isMobile"
				href="https://github.com/KHTangent/cromptch"
			>
				Source Code
			</v-btn>
		</v-container>
	</v-footer>
</template>
<script setup lang="ts">
const isMobile = useDisplay().mobile;
</script>
