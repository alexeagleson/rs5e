export interface ICounter {
  label: string;
  value: string;
  increment: () => void;
  decrement: () => void;
  isLevel: boolean;
}

export const Counter: React.FC<ICounter> = ({
  value,
  label,
  increment,
  decrement,
  isLevel,
}) => {
  return (
    <div
      className="counter"
      style={{
        width: !isLevel ? "180px" : "144px",
        display: "flex",
        justifyContent: "space-evenly",
      }}
    >
      <p style={{ width: "60px" }}>{label}:</p>
      <button onClick={decrement}>-</button>
      <p
        style={{
          width: "72px",
          display: "flex",
          justifyContent: "center",
          fontSize: "16px",
        }}
      >
        {value}
      </p>
      <button onClick={increment}>+</button>
    </div>
  );
};
