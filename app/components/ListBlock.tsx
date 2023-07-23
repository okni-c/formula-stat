interface ListTypes {
  title: string,
  children: any,
}

export default function ListBLock({ children, title }: ListTypes) {
    return (
      <div className="mt-20 p-8 rounded-2xl bg-gradient-to-b dark:from-neutral-900 from-white to-transparent w-full">
        <h3 className="dark:text-white text-black text-3xl font-bold tracking-wide mb-4">{title}</h3>
        {/* Render the children elements */}
        {children}
      </div>
    );
  }
  