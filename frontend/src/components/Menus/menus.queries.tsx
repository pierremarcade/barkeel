import {  useQuery } from "@tanstack/react-query";
import { getMenus } from "@/components/Menus/menus.api";


export const useMenus = () => {
    const { data, refetch } = useQuery({
        queryKey: ['menus'],
        queryFn: getMenus
    })
    const menus = Array.isArray(data)? data : [];
    return { menus, refetch }
}