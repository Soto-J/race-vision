import { PageHeader } from "../components/page-header";

export default function FuelCalculator() {
  return (
    <div>
      <PageHeader
        id="fuel-calculator"
        title="Fuel Calculator"
        description="Fuel calculator keeps track of your average fuel usage per lap and calculate the refuel amount. In a team session, you can keep track of your teammates fuel level, if they also use ioverlay."
        vars={[]}
      />
    </div>
  );
}
