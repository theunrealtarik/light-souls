import React from "react";
import { classNames } from "../lib/utils";

interface InputProps
  extends React.DetailedHTMLProps<
    React.InputHTMLAttributes<HTMLInputElement>,
    HTMLInputElement
  > {}

const Input = React.forwardRef<HTMLInputElement, InputProps>(
  ({ className, ...props }, ref) => (
    <input
      className={classNames(
        "px-5 py-2 focus:ring-2 outline-none ring-neutral-100 transition ring-offset-neutral-900 w-full rounded bg-neutral-700 placeholder:text-neutral-400 text-neutral-100",
        className
      )}
      ref={ref}
      {...props}
    />
  )
);

export default Input;
