export function Button({
  onSubmit,
  disabled = false,
  value,
}: {
  onSubmit: any
  value: string
  disabled?: boolean
}) {
  const handler = (e: any) => {
    e.preventDefault()
    onSubmit(e)
  }

  return (
    <form onSubmit={handler}>
      <input
        className={ButtonStyle()}
        type="submit"
        disabled={disabled}
        value={value}
      />
    </form>
  )
}

export function ButtonStyle() {
  return "rounded bg-stone-600 hover:bg-stone-700 disabled:bg-stone-500 text-white px-4 py-3 cursor-pointer"
}
