export interface IMenuItem{
    id:number,
    label:string,
    href:string
}

export interface IMenu{
    id:number,
    name:string,
    links?:Array<IMenuItem>,
}

export const getMenus = async ():Promise<IMenu[]> =>{
    const res:Response = await fetch(`${process.env.NEXT_PUBLIC_API_HOST}/menus`)

    if(res.ok){
        return await res.json()
    }

    throw new Error('Menus could not be fetched');
}