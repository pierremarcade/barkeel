import Detail from "@/components/Articles/Detail";
import type { Metadata } from 'next'
import { getArticle } from "@/components/Articles/articles.api";
import { notFound } from "next/navigation"
 
type Props = {
    params: { slug: string }
}
   
export async function generateMetadata({ params }: Props): Promise<Metadata|undefined> {
  const slug = params.slug
  try {
    const data = await getArticle(slug);
    return {
      title: data.title,
      description: data.content,
    }
  } catch (erreur) {
    notFound();
  }
}

export default function Page({ params }: { params: { slug: string } }) {
    return (
        <Detail slug={params.slug} />
    );
}