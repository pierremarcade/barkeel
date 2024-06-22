export interface IArticle {
    id: number;
    title: string;
    slug: string;
    section_name: string;
    content: string;
    homepage: boolean;
    description: string;
    published_at: string;
    author_id: number;
}

export const getArticles = async ():Promise<IArticle[]> =>{
    const res:Response = await fetch(`${process.env.NEXT_PUBLIC_API_HOST}/articles`)
    if(res.ok){
        return await res.json()
    }
    throw new Error('Menus could not be fetched');
}

export const getSearchArticles = async (query:string):Promise<IArticle[]> =>{
    const res:Response = await fetch(`${process.env.NEXT_PUBLIC_API_HOST}/articles/search/${query}`)
    if(res.ok){
        return await res.json()
    }
    throw new Error('Articles could not be fetched');
}

export const getArticle = async (slug:string):Promise<IArticle> =>{
    const res:Response = await fetch(`${process.env.NEXT_PUBLIC_API_HOST}/articles/${slug}`)
    if(res.ok){
        return await res.json()
    }
   
    throw new Error('Article could not be fetched');
}