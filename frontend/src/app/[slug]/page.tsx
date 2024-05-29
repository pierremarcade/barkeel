import Detail from "@/components/Articles/Detail";

export default function Page({ params }: { params: { slug: string } }) {
    return (
        <div>
            <Detail slug={params.slug} />
        </div>
    );
}