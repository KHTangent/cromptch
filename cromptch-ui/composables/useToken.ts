export const useToken = () =>
	useState<string>("token", () => {
		const token = useCookie("token");
		token.value = token.value || "";
		return token.value;
	});
