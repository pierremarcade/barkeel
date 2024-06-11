import Detail from "@/components/Articles/Detail";
import { type Metadata } from 'next'

export const metadata: Metadata = {
    title: {
      template: '%s - Docs',
      default: 'Barkeel - Never miss the cache again.',
    },
    description:
      'Coucou.',
  }


export default function Page({ params }: { params: { slug: string } }) {
    return (
        <Detail slug={params.slug} />
    );
}