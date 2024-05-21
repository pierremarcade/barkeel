export interface IMenuItem{
    id:number,
    name:string,
}

export const getMenus = async ():Promise<IMenuItem[]> =>{

    const res:Response = await fetch(`${process.env.API_HOST}/menus`)

    if(res.ok){
        return await res.json()
    }

    throw new Error('Posts could not be fetched');
}