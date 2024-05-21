import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { getMenus } from "@/components/Menus/menus.api";


export const useMenus = () => {
    const { data, refetch } = useQuery({
        queryKey: ['menus'],
        queryFn: getMenus
    })
    return { menus:data, refetch }
}