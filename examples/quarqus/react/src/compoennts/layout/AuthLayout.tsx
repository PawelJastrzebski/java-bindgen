import {
    Card,
    CardHeader
  } from "@nextui-org/react";

import React from 'react';

interface Props {
	title: string;
	children?: React.ReactNode;
}

export const AuthLayout = ({ title = '', children }: Props) => {
	return (
		<Card>
			<CardHeader><h2>{title}</h2></CardHeader>
			<div
				style={{
					display: 'flex',
					flexDirection: 'column',
					justifyContent: 'center',
					alignItems: 'center',
					// height: '100vh',
				}}
			>
				{children}
			</div>
		</Card>
	);
};
