import {useArticle} from "@/components/Articles/articles.queries";
export default function Page({ params }: { params: { slug: string } }) {
    return <div>My Post: {params.slug}</div>
}