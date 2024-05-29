
interface IArticle {
    id: number;
    title: string;
    slug: string;
    content: string;
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

export const getArticle = async (slug:string):Promise<IArticle> =>{
    const res:Response = await fetch(`${process.env.NEXT_PUBLIC_API_HOST}/articles/${slug}`)

    if(res.ok){
        return await res.json()
    }

    throw new Error('Menus could not be fetched');
}