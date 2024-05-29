import {  useQuery } from "@tanstack/react-query";
import { getArticles, getArticle } from "@/components/Articles/articles.api";


export const useArticles = () => {
    const { data, refetch } = useQuery({
        queryKey: ['article'],
        queryFn: getArticles
    })
    const articles = Array.isArray(data)? data : [];
    return { articles, refetch }
}

export const useArticle = (slug:string) => {
    const { data, refetch } = useQuery({
        queryKey: ['article'],
        queryFn: () => getArticle(slug),
    })
    return { data, refetch }
}