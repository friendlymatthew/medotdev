export default function LinkButton({
  title,
  src,
}: {
  title: string;
  src: string;
}) {
  return (
    <a href={src} target="_blank" rel="noreferrer" className="cursor-pointer text-blue-400 font-bold hover:underline italic">
      {title}
    </a>
  );
}
