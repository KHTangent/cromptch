import { Api } from "@/scripts/api";

var apiInstance: Api;

export const useBackend = () => {
	if (!apiInstance) {
		const config = useRuntimeConfig();
		apiInstance = new Api(config.public.apiUrl);
	}
	return apiInstance;
}
