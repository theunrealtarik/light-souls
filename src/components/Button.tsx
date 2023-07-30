import React from "react";
import { classNames } from "@/lib/utils";

interface ButtonProps
  extends React.DetailedHTMLProps<
    React.ButtonHTMLAttributes<HTMLButtonElement>,
    HTMLButtonElement
  > {}

const Button: React.FC<ButtonProps> = ({ children, className, ...props }) => {
  return (
    <button
      className={classNames(
        "px-5 uppercase py-2 bg-neutral-700 font-bold rounded text-neutral-100 focus:ring-2 focus:ring-offset-2 ring-neutral-100 active:scale-95 transition hover:bg-neutral-500 focus:bg-neutral-100 focus:text-neutral-900 ring-offset-neutral-900 outline-none",
        className
      )}
      {...props}
    >
      {children}
    </button>
  );
};

export default Button;
