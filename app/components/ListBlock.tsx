
export default function ListBLock({ children, title }: any) {
    return (
      <div className="mt-20 p-8 rounded-2xl dark:bg-neutral-900 bg-white h-full w-full">
        <h3 className="dark:text-white text-black text-3xl font-bold tracking-wide mb-4">{title}</h3>
        {/* Render the children elements */}
        {children}
      </div>
    );
  }
  