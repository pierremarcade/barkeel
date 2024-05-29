import {  useQuery } from "@tanstack/react-query";
import { getArticles, getArticle } from "@/components/Articles/articles.api";

export const useArticles = () => {
    const { data, refetch } = useQuery({
        queryKey: ['articles'],
        queryFn: getArticles
    })
    const articles = Array.isArray(data)? data : [];
    return { articles, refetch }
}

export const useArticle = (slug:string) => {
    const {  isLoading, isError, data, error, refetch } = useQuery({
        queryKey: ['article', slug],
        queryFn: async () => {
          const data = await getArticle(slug)
          return data
        },
      });

    return { isLoading, isError, data, error, refetch }
}