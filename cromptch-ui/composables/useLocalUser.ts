import type { SelfUser } from "@/scripts/localTypes";

export const useLocalUser = () =>
	useAsyncData<SelfUser | null>(
		"user",
		async () => {
			let token = useToken();
			if (token) {
				let user;
				try {
					user = await useBackend().getSelfUser(token.value);
				} catch (e: unknown) {
					return null;
				}
				return {
					id: user.id,
					username: user.username,
					email: user.email,
					isAdmin: user.isAdmin,
					token: token.value,
				};
			}
			return null;
		},
		{
			watch: [useToken()],
		},
	);
