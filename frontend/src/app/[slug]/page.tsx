import Detail from "@/components/Articles/Detail";

export default function Page({ params }: { params: { slug: string } }) {
    return (
        <Detail slug={params.slug} />
    );
}