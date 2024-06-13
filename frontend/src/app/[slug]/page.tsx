import Detail from "@/components/Articles/Detail";
import type { Metadata, ResolvingMetadata } from 'next'
import { getArticle } from "@/components/Articles/articles.api";
 
type Props = {
    params: { slug: string }
}
   
export async function generateMetadata(
    { params }: Props,
    parent: ResolvingMetadata
  ): Promise<Metadata> {

    const slug = params.slug
    const data = await getArticle(slug);
   
    return {
      title: data.title,
      description: data.content,
    }
  }

export default function Page({ params }: { params: { slug: string } }) {
    return (
        <Detail slug={params.slug} />
    );
}