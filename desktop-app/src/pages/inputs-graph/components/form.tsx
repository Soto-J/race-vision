import { Controller, useForm, SubmitHandler } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { Field, FieldGroup, FieldLabel } from "@/components/ui/field";
import { Switch } from "@/components/ui/switch";

export const InputGraphForm = () => {
  const form = useForm();

  return (
    <form>
      <FieldGroup>
        <Controller
          name="title"
          control={form.control}
          render={({ field, fieldState }) => (
            <Field data-invalid={fieldState.invalid}>
              <FieldLabel></FieldLabel>

              <Switch {...field} id="" />
            </Field>
          )}
        />
      </FieldGroup>
    </form>
  );
};
