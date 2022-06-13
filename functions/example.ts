import { Handler } from "aws-lambda";
import fetch from "node-fetch";

type Parameter = {
  name: string;
  args: string;
  items: ParameterItem[];
};

type ParameterItem = {
  name: string;
  value: string;
};

export const handler: Handler<any, Parameter[]> = async (): Promise<
  Parameter[]
> => {
  console.log("fetching parameters");

  const response = await fetch("http://localhost:3000", {
    method: "GET",
  });

  const parameters = (await response.json()) as Parameter[];
  console.log("fetched parameters: ", JSON.stringify(parameters));

  return parameters;
};
